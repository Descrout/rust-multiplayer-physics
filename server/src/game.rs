use crate::proto::proto_all;
use rapier2d::prelude::*;
use std::collections::HashMap;

const COLORS: [u32; 50] = [
    0xFF6633, 0xFFB399, 0xFF33FF, 0xFFFF99, 0x00B3E6, 0xE6B333, 0x3366E6, 0x999966, 0x99FF99,
    0xB34D4D, 0x80B300, 0x809900, 0xE6B3B3, 0x6680B3, 0x66991A, 0xFF99E6, 0xCCFF1A, 0xFF1A66,
    0xE6331A, 0x33FFCC, 0x66994D, 0xB366CC, 0x4D8000, 0xB33300, 0xCC80CC, 0x66664D, 0x991AFF,
    0xE666FF, 0x4DB3FF, 0x1AB399, 0xE666B3, 0x33991A, 0xCC9999, 0xB3B31A, 0x00E680, 0x4D8066,
    0x809980, 0xE6FF80, 0x1AFF33, 0x999933, 0xFF3380, 0xCCCC00, 0x66E64D, 0x4D80CC, 0x9900B3,
    0xE64D66, 0x4DB380, 0xFF4D4D, 0x99E6E6, 0x6666FF,
];

const WIDTH: f32 = 960.0;
const HEIGHT: f32 = 540.0;

// PIXELS PER METER
const PPM: f32 = 50.0;

struct Rect {
    id: usize,
    w: f32,
    h: f32,
    r_handle: RigidBodyHandle,
}

pub struct Game {
    players: HashMap<u32, proto_all::Entity>,
    rects: Vec<Rect>,

    // rapier.rs stuff
    islands: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    bodies: RigidBodySet,
    colliders: ColliderSet,
    joints: JointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
    physics_pipeline: PhysicsPipeline,
    integration_parameters: IntegrationParameters,
    gravity: Vector<f32>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            rects: Vec::new(),

            // rapier.rs stuff
            gravity: vector![0.0, -9.81],
            integration_parameters: IntegrationParameters::default(),
            islands: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            joints: JointSet::new(),
            ccd_solver: CCDSolver::new(),
            bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            query_pipeline: QueryPipeline::new(),
            physics_pipeline: PhysicsPipeline::new(),
        }
    }

    pub fn init(&mut self) {
        /* Create the static walls. */
        self.add_cuboid(0.0, HEIGHT - 10.0, WIDTH, 10.0, 0.0, true);
        self.add_cuboid(0.0, 0.0, WIDTH, 10.0, 0.0, true);

        self.add_cuboid(0.0, 0.0, 10.0, HEIGHT, 0.0, true);
        self.add_cuboid(WIDTH - 10.0, 0.0, 10.0, HEIGHT, 0.0, true);

        self.add_cuboid(WIDTH / 4.0, HEIGHT / 2.0, WIDTH / 2.0, 10.0, 0.3, true);

        /* Create the cuboids. */
        let angle = std::f32::consts::PI / 4.0;
        for i in 0..9 {
            let i = i as f32;
            self.add_cuboid(50.0 + 100.0 * i, 100.0, 70.0, 70.0, angle + 0.3 * i, false);
        }

        for i in 0..11 {
            let i = i as f32;
            self.add_cuboid(50.0 + 80.0 * i, 290.0, 50.0, 50.0, angle + 0.3 * i, false);
        }

        for i in 0..10 {
            let i = i as f32;
            self.add_cuboid(50.0 + 80.0 * i, 480.0, 30.0, 30.0, angle + 0.3 * i, false);
        }
    }

    fn add_cuboid(&mut self, x: f32, y: f32, w: f32, h: f32, angle: f32, is_static: bool) {
        let id = self.rects.len();

        // RigidBody
        let r_builder = if is_static {
            RigidBodyBuilder::new_static()
        } else {
            RigidBodyBuilder::new_dynamic()
        };

        let w = (w / PPM) / 2.0;
        let h = (h / PPM) / 2.0;

        let r_body = r_builder
            .translation(vector![(x / PPM) + w, ((HEIGHT - y) / PPM) - h])
            .rotation(angle)
            .user_data(COLORS[id % 50] as u128) // I'm using user_data to present cuboid's color.
            .build();

        // Collider
        let c_builder = ColliderBuilder::cuboid(w, h);
        let collider = if is_static {
            c_builder.build()
        } else {
            c_builder.restitution(0.7).build()
        };

        let r_handle = self.bodies.insert(r_body);
        self.colliders
            .insert_with_parent(collider, r_handle, &mut self.bodies);

        self.rects.push(Rect { id, r_handle, w, h });
    }

    pub fn add_player(&mut self, id: u32) {
        self.players.insert(
            id,
            proto_all::Entity {
                id: id,
                x: 0.0,
                y: 0.0,
                pressed: false,
                color: format!("#{:X}", COLORS[id as usize % 50]),
            },
        );
    }

    pub fn remove_player(&mut self, id: u32) {
        self.players.remove(&id);
    }

    pub fn set_input(&mut self, id: u32, input: proto_all::GameInput) {
        let mut player = self.players.get_mut(&id).unwrap();
        player.x = input.x;
        player.y = input.y;
        player.pressed = input.pressed;
    }

    pub fn update(&mut self) {
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.islands,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joints,
            &mut self.ccd_solver,
            &(),
            &(),
        );

        // Update the query pipeline first
        self.query_pipeline
            .update(&self.islands, &self.bodies, &self.colliders);

        // Initialize a vec for holding colliders that intersects with player's mouse pointers.
        let mut collider_handles = Vec::new();
        for (_, player) in self.players.iter() {
            // If a player does not click with the mouse, we pass.
            if !player.pressed {
                continue;
            }

            // Convert player mouse positions to rapier physics positions.
            let point = point![player.x / PPM, (HEIGHT - player.y) / PPM];

            // Check intersection and push those who intersects into the collider_handles vec.
            self.query_pipeline.intersections_with_point(
                &self.colliders,
                &point,
                InteractionGroups::all(),
                None,
                |handle| {
                    collider_handles.push((handle, point));
                    false // Make this true, to make players affect multiple cuboids.
                },
            );
        }

        // Apply force and impulse to the intersected boxes.
        for (handle, point) in collider_handles.into_iter() {
            let collider = self.colliders.get(handle).unwrap();
            let body = self.bodies.get_mut(collider.parent().unwrap()).unwrap();
            if body.is_static() {
                continue;
            }
            let x = body.translation().x;
            let y = body.translation().y;

            body.apply_force(vector![0.0, 9.81], true);
            body.apply_impulse_at_point(vector![(x - point.x), (y - point.y) * 3.0], point, true);
            body.apply_impulse(vector![(x - point.x), (y - point.y).abs() * 3.0], true);
        }
    }

    pub fn get_state(&self) -> proto_all::State {
        let mut state = proto_all::State {
            entities: Vec::new(),
            bodies: Vec::new(),
        };

        for (_, entity) in self.players.iter() {
            state.entities.push(entity.clone());
        }

        for rect in self.rects.iter() {
            let body = self.bodies.get(rect.r_handle).unwrap();
            let pos = body.translation();
            state.bodies.push(proto_all::Body {
                id: rect.id as u32,
                color: format!("#{:X}", body.user_data),
                x: pos.x,
                y: pos.y,
                w: rect.w,
                h: rect.h,
                rotation: body.rotation().angle(),
            });
        }

        state
    }
}
