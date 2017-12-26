/*
* Copyright 2017 Michal Mauser
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU Affero General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU Affero General Public License for more details.
*
* You should have received a copy of the GNU Affero General Public License
* along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use rand::Rng;
use rand::ThreadRng;
use resource_mng::Instance;

pub struct RunResult {
    pub function_number: u8,
    pub name: String,
    pub amount: usize,
    pub material_id: String,
}

pub fn run(instance: &mut Instance, rng: &mut ThreadRng, max_values: usize) -> Result<RunResult, &'static str> {
    match rng.gen::<u8>() % 4 {
        0 => { //add material
            let name = rng.gen::<u16>().to_string();
            let supply = rng.gen::<usize>() % max_values;
            if instance.add_material(name.clone(), supply) {
                Ok(RunResult {
                    function_number: 0,
                    name,
                    amount: supply,
                    material_id: String::new()
                })
            } else { Err("Could not add material") }
        }
        1 => { // add product
            let name = rng.gen::<u16>().to_string();
            let material_amount = rng.gen::<usize>() % max_values /64;
            let rnd_index = rng.gen::<usize>() % instance.get_material_count();
            let material_id = instance.tst_get_materials().iter().enumerate()
                .nth(rnd_index)
                .unwrap()
                .1
                .0.clone();
            let work_complexity = rng.gen::<u8>();
            if instance.add_product(name.clone(), material_id.clone(), work_complexity, material_amount) {
                Ok(RunResult {
                    function_number: 1,
                    name,
                    amount: material_amount,
                    material_id
                })
            } else { Err("Could not add product") }
        }
        2 => { // order product
            let amount = rng.gen::<usize>() % max_values /64;
            let product_count = instance.get_product_count();
            let rnd_index = if product_count > 0 {
                rng.gen::<usize>() % product_count
            } else { return Err("Product count cannot be zero") };
            if rnd_index >= product_count {panic!("index is larger than product count")}
            let name = instance.tst_get_products().iter().enumerate()
                .nth(rnd_index)
                .unwrap()
                .1
                .0.clone();
            let tmp1; let tmp2;
            {let tmp = &instance.get_product_types(&name).material_amount;
            tmp1 = tmp.0.clone();
            tmp2 = tmp.1;} //fix me
            match instance.order_product(&name, amount) {
                Ok(_) => {
                    //let (tmp, tmp1) = instance.get_product_types(&name).material_amount.clone_into();
                    Ok(RunResult {
                    function_number: 2,
                    name: name.clone(),
                    amount: amount * tmp2,
                    material_id: tmp1,
                })},
                Err(e) => {
                    Err(e)
                },
            }
        }
        3 => { // update supply
            let amount = rng.gen::<usize>() % max_values;
            let material_count = instance.get_material_count();
            let rnd_index = if material_count > 0 {
                rng.gen::<usize>() % material_count
            } else { return Err("Material count cannot be zero") };
            let name = instance.tst_get_materials().iter().enumerate()
                .nth(rnd_index)
                .unwrap()
                .1
                .0.clone();
            if instance.update_supply(&name, amount) {
                Ok(RunResult {
                    function_number: 3,
                    name,
                    amount,
                    material_id: String::new(),
                })
            } else { Err("Supply update failed") }
        }
        _ => Err("No such function")
    }
}