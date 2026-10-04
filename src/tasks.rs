use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use cu29::prelude::{CuMsg};
use std::sync::Arc;
use cu29::config::ComponentConfig;
use cu29::{CuResult};
use cu29::context::CuContext;
use crate::constants;
use rand::Rng;

const _: f64 = constants::speed;

#[derive(cu29::prelude::Reflect)]
pub struct Localization {
    // location task
    speed: f64,  // the move speed
    rand_movement: (f64, f64),
    rand_direction: (bool, bool)
}

#[derive(cu29::prelude::Reflect)]
pub struct Movement {
    // move task

}

impl cu29::prelude::Freezable for Movement {

}

impl cu29::prelude::Freezable for Localization {

}

impl cu29::prelude::CuSrcTask for Localization {
    type Resources<'r> = ();
    type Output<'m> = cu29::prelude::output_msg!(crate::msgs::Location);

    fn new(_onfig: Option<&cu29::prelude::ComponentConfig>, resources: Self::Resources<'_>) -> cu29::prelude::CuResult<Self>
    where
        Self: Sized,
    {
        let speed: f64 = constants::speed;

        Ok(Self {
            speed: speed,
            rand_movement: (0.0, 0.0),
            rand_direction: (false, false)
        })
    }

    fn preprocess(&mut self, _ctx: &cu29::prelude::CuContext) -> cu29::prelude::CuResult<()> {
        // randomly move direction alone x, y axis
        self.rand_direction.0 = rand::random();
        self.rand_direction.1 = rand::random();
        // random move distance (0, 1) * speed
        self.rand_movement.0 = rand::random::<f64>();
        self.rand_movement.1 = rand::random::<f64>();

        Ok(())
    }

    fn process(&mut self, ctx: &cu29::prelude::CuContext, output: &mut Self::Output<'_>) -> cu29::prelude::CuResult<()> {

        if self.rand_movement.0 != 0.0 && self.rand_movement.1 != 1.0 {
            // randomly move x distance alone positive/ negative axis
            if self.rand_direction.0 && self.rand_direction.1 {
                output.set_payload(
                    crate::msgs::Location::new(
                        self.rand_movement.0 * self.speed,
                        self.rand_movement.1 * self.speed)
                );
            }
            else if (!self.rand_direction.0 && self.rand_direction.1) {
                output.set_payload(
                    crate::msgs::Location::new(
                        self.rand_movement.0 * self.speed * -1.0,
                        self.rand_movement.1 * self.speed)
                );
            }
            else if self.rand_direction.0 && !self.rand_direction.1 {
                output.set_payload(
                    crate::msgs::Location::new(
                        self.rand_movement.0 * self.speed,
                        self.rand_movement.1 * self.speed * -1.0)
                );
            }
            else {
                output.set_payload(
                    crate::msgs::Location::new(
                        self.rand_movement.0 * self.speed * -1.0,
                        self.rand_movement.1 * self.speed * -1.0)
                );
            }
            // reset movement
            self.rand_movement.0 = 0.0;
            self.rand_movement.1 = 0.0;

        }
        else {
            // random value is empty
        }

        Ok(())
    }
}

impl cu29::prelude::CuSinkTask for Movement {
    type Resources<'r> = ();
    type Input<'m> = cu29::prelude::input_msg!(crate::msgs::Location);

    fn new(config: Option<&cu29::prelude::ComponentConfig>, resources: Self::Resources<'_>) -> cu29::prelude::CuResult<Self>
    where
        Self: Sized,

    {
        Ok(Self {})
    }

    fn process(&mut self, ctx: &cu29::prelude::CuContext, input: &Self::Input<'_>) -> cu29::prelude::CuResult<()> {
        input.payload().unwrap().show();
        // debug!("{}", input.payload().unwrap());
        Ok(())
    }
}
