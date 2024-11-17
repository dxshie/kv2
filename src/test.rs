#[cfg(test)]
mod tests {
    use crate::parse_kv2;
    use log::{error, info};

    #[test]
    fn parse_vk2_test_1() {
        let input = r#"
<!-- DMXVersion keyvalues2_v1 -->
"DmePresetGroup"
{
	"id" "elementid" "2b77ac04-3f32-46eb-a2a7-80f1d6d9872e"
	"name" "string" "phoneme"
	"readonly" "bool" "0"
	"visible" "bool" "1"
	"presets" "element_array" 
	[
		"DmePreset"
		{
			"id" "elementid" "117e1a71-d867-4857-b404-6651f2cdd68a"
			"name" "string" "p_silence"
			"controlValues" "element_array" 
			[
				"DmeElement"
				{
					"id" "elementid" "7a0ba9b3-5434-43fe-94ed-068ced2351e1"
					"balance" "float" "0.5"
					"midpoint" "float" "0"
					"value" "float" "0"
					"name" "string" "CloseLidUp"
				},
				"DmeElement"
				{
					"id" "elementid" "e707ed99-f2f2-4a9a-844d-fc8f3d1eda36"
					"balance" "float" "0.5"
					"midpoint" "float" "0"
					"value" "float" "0"
					"name" "string" "CloseLidLo"
				},
				"DmeElement"
				{
					"id" "elementid" "ae60ee94-7c9a-494c-be01-eba193e90146"
					"balance" "float" "0.5"
					"midpoint" "float" "0"
					"value" "float" "0"
					"name" "string" "InnerSquint"
				}
			]
		}
	]
}
"#;
        match parse_kv2(input) {
            Ok(data) => {
                info!("data {:?}", data);
            }
            Err(e) => {
                error!("{:?}", e);
                panic!("expected the test: test to pass")
            }
        }
    }
    #[test]
    pub fn parse_vk2_test_2() {
        let input = r#"
<!-- DMXVersion keyvalues2_v1 -->
"DmeElement"
{
	"id" "elementid" "833dbad4-0848-4c77-a49c-5a702a545c55"
	"name" "string" "untitled"
	"particleSystemDefinitions" "element_array" 
	[
		"DmeParticleSystemDefinition"
		{
			"id" "elementid" "3535d7f5-7d31-4b97-b772-46fadd300992"
			"name" "string" "default"
			"material" "string" "effects\\yellowflare.vmt"
			"children" "element_array" 
			[
			]
			"color" "color" "255 255 255 255"
			"operators" "element_array" 
			[
				"DmeParticleOperator"
				{
					"id" "elementid" "a66571b0-1657-41ad-a160-ba5c3b722835"
					"name" "string" "alpha_fade"
					"functionName" "string" "alpha_fade"
					"operator start fadein" "float" "0"
					"operator end fadein" "float" "0"
					"operator start fadeout" "float" "0"
					"operator end fadeout" "float" "0"
					"start_alpha" "float" "1"
					"end_alpha" "float" "0"
					"start_fade_in_time" "float" "0"
					"end_fade_in_time" "float" "0.5"
					"start_fade_out_time" "float" "0.5"
					"end_fade_out_time" "float" "1"
				},
				"DmeParticleOperator"
				{
					"id" "elementid" "1ec8a22e-5e14-45fe-9dab-02ffdd5772c8"
					"name" "string" "basic_movement"
					"functionName" "string" "basic_movement"
					"operator start fadein" "float" "0"
					"operator end fadein" "float" "0"
					"operator start fadeout" "float" "0"
					"operator end fadeout" "float" "0"
					"gravity" "vector3" "0 0 0"
					"drag" "float" "0"
					"max constraint passes" "int" "3"
				}
			]
			"renderers" "element_array" 
			[
				"DmeParticleOperator"
				{
					"id" "elementid" "e92d1bc3-7c85-41a4-9779-ca71ea83c139"
					"name" "string" "render_animated_sprites"
					"functionName" "string" "render_animated_sprites"
					"operator start fadein" "float" "0"
					"operator end fadein" "float" "0"
					"operator start fadeout" "float" "0"
					"operator end fadeout" "float" "0"
					"animation rate" "float" "0.1000000015"
					"animation_fit_lifetime" "bool" "0"
					"orientation_type" "int" "0"
				}
			]
			"initializers" "element_array" 
			[
				"DmeParticleOperator"
				{
					"id" "elementid" "7415690d-8459-438c-ae78-8ab6c1320095"
					"name" "string" "lifetime_random"
					"functionName" "string" "lifetime_random"
					"operator start fadein" "float" "0"
					"operator end fadein" "float" "0"
					"operator start fadeout" "float" "0"
					"operator end fadeout" "float" "0"
					"lifetime_min" "float" "1"
					"lifetime_max" "float" "1"
					"lifetime_random_exponent" "float" "1"
				},
				"DmeParticleOperator"
				{
					"id" "elementid" "bfe40106-301d-4024-bb24-879162b63418"
					"name" "string" "position_within_sphere"
					"functionName" "string" "position_within_sphere"
					"operator start fadein" "float" "0"
					"operator end fadein" "float" "0"
					"operator start fadeout" "float" "0"
					"operator end fadeout" "float" "0"
					"distance_min" "float" "0"
					"distance_max" "float" "0"
					"control_point_number" "int" "0"
					"speed_min" "float" "12"
					"speed_max" "float" "32"
					"speed_random_exponent" "float" "1"
					"speed_in_local_coordinate_system_min" "vector3" "0 0 0"
					"speed_in_local_coordinate_system_max" "vector3" "0 0 0"
				}
			]
			"emitters" "element_array" 
			[
				"DmeParticleOperator"
				{
					"id" "elementid" "9c8b45bf-2644-4c8f-9a37-0873b578aeb7"
					"name" "string" "emit_continuously"
					"functionName" "string" "emit_continuously"
					"operator start fadein" "float" "0"
					"operator end fadein" "float" "0"
					"operator start fadeout" "float" "0"
					"operator end fadeout" "float" "0"
					"emission_start_time" "float" "0"
					"emission_rate" "float" "100"
					"emission_duration" "float" "0"
				}
			]
			"forces" "element_array" 
			[
			]
			"constraints" "element_array" 
			[
			]
			"preventNameBasedLookup" "bool" "0"
			"max_particles" "int" "1000"
			"initial_particles" "int" "0"
			"bounding_box_control_point" "int" "0"
			"bounding_box_min" "vector3" "-10 -10 -10"
			"bounding_box_max" "vector3" "10 10 10"
			"radius" "float" "5"
			"rotation" "float" "0"
			"rotation_speed" "float" "0"
			"sequence_number" "int" "0"
		}
	]
}
            "#;

        match parse_kv2(input) {
            Ok(data) => {
                info!("data {:?}", data);
            }
            Err(e) => {
                error!("{:?}", e);
                panic!("expected the test: test2 to pass")
            }
        }
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_tests {
    use crate::kv2_serde::serde_kv2;
    use log::{error, info};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct TestOne {
        id: String,
        name: String,
        readonly: bool,
        visible: bool,
    }

    #[test]
    fn serde_parse_vk2_test_1() {
        let input = r#"
<!-- DMXVersion keyvalues2_v1 -->
"DmePresetGroup"
{
	"id" "elementid" "2b77ac04-3f32-46eb-a2a7-80f1d6d9872e"
	"name" "string" "phoneme"
	"readonly" "bool" "0"
	"visible" "bool" "1"
	"presets" "element_array" 
	[
		"DmePreset"
		{
			"id" "elementid" "117e1a71-d867-4857-b404-6651f2cdd68a"
			"name" "string" "p_silence"
			"controlValues" "element_array" 
			[
				"DmeElement"
				{
					"id" "elementid" "7a0ba9b3-5434-43fe-94ed-068ced2351e1"
					"balance" "float" "0.5"
					"midpoint" "float" "0"
					"value" "float" "0"
					"name" "string" "CloseLidUp"
				},
				"DmeElement"
				{
					"id" "elementid" "e707ed99-f2f2-4a9a-844d-fc8f3d1eda36"
					"balance" "float" "0.5"
					"midpoint" "float" "0"
					"value" "float" "0"
					"name" "string" "CloseLidLo"
				},
				"DmeElement"
				{
					"id" "elementid" "ae60ee94-7c9a-494c-be01-eba193e90146"
					"balance" "float" "0.5"
					"midpoint" "float" "0"
					"value" "float" "0"
					"name" "string" "InnerSquint"
				}
			]
		}
	]
}
"#;
        match serde_kv2::<TestOne>(input) {
            Ok(data) => {
                info!("data {:?}", data);
            }
            Err(e) => {
                error!("{:?}", e);
                panic!("expected the test: test to pass")
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct TestTwo {
        id: String,
        name: String,
    }

    #[test]
    pub fn serde_parse_vk2_test_2() {
        let input = r#"
<!-- DMXVersion keyvalues2_v1 -->
"DmeElement"
{
	"id" "elementid" "833dbad4-0848-4c77-a49c-5a702a545c55"
	"name" "string" "untitled"
	"particleSystemDefinitions" "element_array" 
	[
		"DmeParticleSystemDefinition"
		{
			"id" "elementid" "3535d7f5-7d31-4b97-b772-46fadd300992"
			"name" "string" "default"
			"material" "string" "effects\\yellowflare.vmt"
			"children" "element_array" 
			[
			]
			"color" "color" "255 255 255 255"
			"operators" "element_array" 
			[
				"DmeParticleOperator"
				{
					"id" "elementid" "a66571b0-1657-41ad-a160-ba5c3b722835"
					"name" "string" "alpha_fade"
					"functionName" "string" "alpha_fade"
					"operator start fadein" "float" "0"
					"operator end fadein" "float" "0"
					"operator start fadeout" "float" "0"
					"operator end fadeout" "float" "0"
					"start_alpha" "float" "1"
					"end_alpha" "float" "0"
					"start_fade_in_time" "float" "0"
					"end_fade_in_time" "float" "0.5"
					"start_fade_out_time" "float" "0.5"
					"end_fade_out_time" "float" "1"
				},
				"DmeParticleOperator"
				{
					"id" "elementid" "1ec8a22e-5e14-45fe-9dab-02ffdd5772c8"
					"name" "string" "basic_movement"
					"functionName" "string" "basic_movement"
					"operator start fadein" "float" "0"
					"operator end fadein" "float" "0"
					"operator start fadeout" "float" "0"
					"operator end fadeout" "float" "0"
					"gravity" "vector3" "0 0 0"
					"drag" "float" "0"
					"max constraint passes" "int" "3"
				}
			]
			"renderers" "element_array" 
			[
				"DmeParticleOperator"
				{
					"id" "elementid" "e92d1bc3-7c85-41a4-9779-ca71ea83c139"
					"name" "string" "render_animated_sprites"
					"functionName" "string" "render_animated_sprites"
					"operator start fadein" "float" "0"
					"operator end fadein" "float" "0"
					"operator start fadeout" "float" "0"
					"operator end fadeout" "float" "0"
					"animation rate" "float" "0.1000000015"
					"animation_fit_lifetime" "bool" "0"
					"orientation_type" "int" "0"
				}
			]
			"initializers" "element_array" 
			[
				"DmeParticleOperator"
				{
					"id" "elementid" "7415690d-8459-438c-ae78-8ab6c1320095"
					"name" "string" "lifetime_random"
					"functionName" "string" "lifetime_random"
					"operator start fadein" "float" "0"
					"operator end fadein" "float" "0"
					"operator start fadeout" "float" "0"
					"operator end fadeout" "float" "0"
					"lifetime_min" "float" "1"
					"lifetime_max" "float" "1"
					"lifetime_random_exponent" "float" "1"
				},
				"DmeParticleOperator"
				{
					"id" "elementid" "bfe40106-301d-4024-bb24-879162b63418"
					"name" "string" "position_within_sphere"
					"functionName" "string" "position_within_sphere"
					"operator start fadein" "float" "0"
					"operator end fadein" "float" "0"
					"operator start fadeout" "float" "0"
					"operator end fadeout" "float" "0"
					"distance_min" "float" "0"
					"distance_max" "float" "0"
					"control_point_number" "int" "0"
					"speed_min" "float" "12"
					"speed_max" "float" "32"
					"speed_random_exponent" "float" "1"
					"speed_in_local_coordinate_system_min" "vector3" "0 0 0"
					"speed_in_local_coordinate_system_max" "vector3" "0 0 0"
				}
			]
			"emitters" "element_array" 
			[
				"DmeParticleOperator"
				{
					"id" "elementid" "9c8b45bf-2644-4c8f-9a37-0873b578aeb7"
					"name" "string" "emit_continuously"
					"functionName" "string" "emit_continuously"
					"operator start fadein" "float" "0"
					"operator end fadein" "float" "0"
					"operator start fadeout" "float" "0"
					"operator end fadeout" "float" "0"
					"emission_start_time" "float" "0"
					"emission_rate" "float" "100"
					"emission_duration" "float" "0"
				}
			]
			"forces" "element_array" 
			[
			]
			"constraints" "element_array" 
			[
			]
			"preventNameBasedLookup" "bool" "0"
			"max_particles" "int" "1000"
			"initial_particles" "int" "0"
			"bounding_box_control_point" "int" "0"
			"bounding_box_min" "vector3" "-10 -10 -10"
			"bounding_box_max" "vector3" "10 10 10"
			"radius" "float" "5"
			"rotation" "float" "0"
			"rotation_speed" "float" "0"
			"sequence_number" "int" "0"
		}
	]
}
            "#;

        match serde_kv2::<TestTwo>(input) {
            Ok(data) => {
                info!("data {:?}", data);
            }
            Err(e) => {
                error!("{:?}", e);
                panic!("expected the test: test2 to pass")
            }
        }
    }
}
