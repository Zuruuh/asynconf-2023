use domain::{
    calculator::Calculator,
    vehicle::{Energy, Generation, Mileage, Vehicle},
    Config,
};
use leptos::{
    component, create_signal, event_target_value, mount_to_body, view, For, IntoView, Show,
    SignalWith, SignalWithUntracked,
};

fn main() {
    let _ = console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App /> })
}

#[component]
fn App() -> impl IntoView {
    let (config, _) = create_signal(get_default_config());
    let calculator = move || Calculator::new(config());

    let (car_model, set_car_model) = create_signal(
        config.with_untracked(|config| config.vehicle_data.first().unwrap().name.clone()),
    );

    let (energy, set_energy) = create_signal(
        config.with_untracked(|config| config.energy_data.first().unwrap().name.clone()),
    );
    let (mileage, set_mileage) = create_signal(0);
    let (generation, set_generation) = create_signal(0);
    let (passengers_count, set_passengers_count) = create_signal(0);

    let (calculated_rate, set_calculated_rate) = create_signal(Option::<f32>::None);

    view! {
        <main>
            <h1>"AXA Green bank"</h1>
            <h2>"Calculateur de taux d'emprunt pour véhicules écologiques"</h2>
            <Show when=move || calculated_rate().is_some()>
                <hr />
                <h4>"Vos intérêts sont estimés à: " {move || calculated_rate().unwrap()}</h4>
            </Show>
            <form on:submit=move |event| {
                event.prevent_default();

                let vehicle = Vehicle::new(
                    car_model(),
                    Energy(energy()),
                    Mileage(mileage()),
                    Generation(generation())
                );

                set_calculated_rate(Some(calculator().calculate_rate(vehicle, passengers_count())))
            }>
                <label for="car-name">
                    Modèle
                    <select id="car-name" prop:value=car_model on:input=move |event| {
                        set_car_model(event_target_value(&event));
                    }>
                        <For
                            each=move || config.with(|config| config.vehicle_data.clone())
                            key=|vehicle_data| vehicle_data.name.clone()
                            children=move |vehicle_data| view! {
                                <option prop:value={&vehicle_data.name}>{vehicle_data.name}</option>
                            }
                        />
                    </select>
                </label>

                <label for="energy">
                    "Source d'énergie"
                    <select id="energy" prop:value=energy on:input=move |event| {
                        set_energy(event_target_value(&event));
                    }>
                        <For
                            each=move || config.with(|config| config.energy_data.clone())
                            key=|energy_data| energy_data.name.clone()
                            children=move |energy_data| view! {
                                <option prop:value={&energy_data.name}>{energy_data.name}</option>
                            }
                        />
                    </select>
                </label>

                <label for="mileage">
                    Kilométrage
                    <input
                        type="number"
                        id="mileage"
                        step="100"
                        placeholder="15000"
                        required
                        prop:value={mileage}
                        on:input=move |event| {
                            set_mileage(event_target_value(&event).parse::<usize>().unwrap_or_default());
                        }
                    />
                </label>

                <label for="generation">
                    Année de production
                    <input
                        id="generation"
                        type="number"
                        placeholder="2005"
                        required
                        min="1900"
                        max="2023"
                        prop:value={generation}
                        on:input=move |event| {
                            set_generation(event_target_value(&event).parse::<usize>().unwrap_or_default());
                        }
                    />
                </label>

                <label for="passagers">
                    Nombre de passagers
                    <input
                        type="number"
                        id="passagers"
                        placeholder="2"
                        required
                        min="1"
                        max="4"
                        prop:value={passengers_count}
                        on:input=move |event| {
                            set_passengers_count(event_target_value(&event).parse::<u8>().unwrap_or_default());
                        }
                    />
                </label>

                <button type="submit">Lancer la simulation</button>
            </form>
        </main>
    }
}

fn get_default_config() -> Config {
    serde_json::from_str(std::include_str!("../../../data.json")).unwrap()
}
