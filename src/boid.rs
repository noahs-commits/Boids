use rand::Rng;


const SPEED: f32=0.5;
const RESTORING_FORCE_START: f32=0.1;
const RESTORING_FORCE: f32=10.0;

const SEPARATION_DIST: f32=0.1;
const SEPARATION_FORCE: f32=200.0;

const CLOSE_DIST: f32=0.1;
const ALIGNMENT_FORCE: f32=0.5;




pub struct Boid{
    pub pos: [f32;2],
    pub vel: [f32;2],
}

impl Boid{
    pub fn normalise_vel(&mut self){
        let vel=(self.vel[0]*self.vel[0])+(self.vel[1]*self.vel[1]);

        let mult=vel.powf(-0.5);

        self.vel[0]*=mult;
        self.vel[1]*=mult;
    }
    pub fn update(boids: &mut Vec<Boid>, dt: f32){

        //do not go off the screen
        boids.iter_mut().for_each(|b| {

            let left_force=(b.pos[0]-1.0)*(-RESTORING_FORCE/(RESTORING_FORCE_START-1.0))+RESTORING_FORCE;
            if left_force>0.0{
                b.vel[0]-=left_force*dt;
            }

            let right_force=((-b.pos[0])-1.0)*(-RESTORING_FORCE/(RESTORING_FORCE_START-1.0))+RESTORING_FORCE;
            if right_force>0.0{
                b.vel[0]+=right_force*dt;
            }

            let up_force=(b.pos[1]-1.0)*(-RESTORING_FORCE/(RESTORING_FORCE_START-1.0))+RESTORING_FORCE;
            if up_force>0.0{
                b.vel[1]-=up_force*dt;
            }

            let down_force=((-b.pos[1])-1.0)*(-RESTORING_FORCE/(RESTORING_FORCE_START-1.0))+RESTORING_FORCE;
            if down_force>0.0{
                b.vel[1]+=down_force*dt;
            }
        });

        let len = boids.len();
        for i in 0..len {
            for j in i + 1..len {
                let difference=[boids[i].pos[0]-boids[j].pos[0],boids[i].pos[1]-boids[j].pos[1]];
                let distence=(difference[0].powi(2)+difference[1].powi(2)).sqrt();
                //linear function if distence==0 return SEPARATION_FORCE
                //if distence==SEPARATION_DIST return 0
                //slope=rise/run=-SEPARATION_FORCE/SEPARATION_DIST
                let force=distence*(-SEPARATION_FORCE/SEPARATION_DIST)+SEPARATION_FORCE;

                if (force>0.0){
                    let acceleration=force*dt;
                    let force_vec=[difference[0]*acceleration,difference[1]*acceleration];
                    //println!("{:?} {:?} {:?}",force,force_vec,difference);

                    boids[i].vel[0]+=force_vec[0];
                    boids[i].vel[1]+=force_vec[1];
                    boids[j].vel[0]-=force_vec[0];
                    boids[j].vel[1]-=force_vec[1];
                }
            }
        }

        for i in 0..len {
            for j in i + 1..len {
                let difference=[boids[i].pos[0]-boids[j].pos[0],boids[i].pos[1]-boids[j].pos[1]];
                let distence=(difference[0].powi(2)+difference[1].powi(2)).sqrt();

                if distence<CLOSE_DIST{

                    let (mut x1, mut y1, mut x2, mut y2)=(boids[i].vel[0],boids[i].vel[1],boids[j].vel[0],boids[j].vel[1]);

                    x1+=boids[j].vel[0]*ALIGNMENT_FORCE*dt;
                    y1+=boids[j].vel[1]*ALIGNMENT_FORCE*dt;
                    x2+=boids[i].vel[0]*ALIGNMENT_FORCE*dt;
                    y2+=boids[i].vel[1]*ALIGNMENT_FORCE*dt;

                    boids[i].vel=[x1,y1];
                    boids[j].vel=[x2,y2];
                }
            }
        }

        boids.iter_mut().for_each(|b| {
            b.normalise_vel();
        });

        boids.iter_mut().for_each(|b| {
            b.pos[0]+=b.vel[0]*SPEED*dt;
            b.pos[1]+=b.vel[1]*SPEED*dt;
        });
    }
    pub fn get_starting_boids()->Vec<Boid>{
        let mut output=Vec::new();
        let mut rng = rand::thread_rng();    



        for _ in 0..250{
            let pos=[rng.gen_range(-1.0..1.0),rng.gen_range(-1.0..1.0)];
            let angle=rng.gen_range(0.0..std::f32::consts::TAU);
            let vel=angle.sin_cos().into();

            output.push(Boid{ pos: pos, vel: vel})
        }

        return output;
    }   
}