#[test]
fn split_folders_files() {
    use super::data::{Data};

    let mut some_data = Data::new("");
    let mut vec = vec![];
    vec.push("/folder/folder_2/folder_3/some_file.txt".to_owned());
    vec.push("/folder/folder_2/folder_3".to_owned());
    vec.push("/folder/folder_2/folder_3/".to_owned());
    vec.push("/folder/folder_2/folder_3/test_again/with_folder".to_owned());
    vec.push("/folder/folder_2".to_owned());
    vec.push("/folder/folder_test/folder_3/some_file_2.txt".to_owned());

    some_data.remove = Some(vec);;

    let mut test_data_folders = vec![];
    test_data_folders.push("/folder/folder_2/folder_3".to_owned());
    test_data_folders.push("/folder/folder_2/folder_3/".to_owned());
    test_data_folders.push("/folder/folder_2/folder_3/test_again/with_folder".to_owned());
    test_data_folders.push("/folder/folder_2".to_owned());

    let mut test_data_files = vec![];
    test_data_files.push("/folder/folder_2/folder_3/some_file.txt".to_owned());
    test_data_files.push("/folder/folder_test/folder_3/some_file_2.txt".to_owned());

    assert_eq!(some_data.split_folders_files(), (test_data_folders, test_data_files));
}

#[test]
fn filter_remove() {
    use super::data::{Data};

    let mut some_data = Data::new("");

    let temp_vec = [
        "/app_canada_layer/cartridge/static/default/dist/css/styleguide.css",
        "/app_canada_layer/cartridge/static/default/dist/css/commons.css.map",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/system.config.js",
        "/app_canada_layer/cartridge/static/default/dist/css/styleguide.css.map",
        "/app_canada_layer/cartridge/static/default/dist/css/commons.css",
        "/app_canada_layer/cartridge/static/default/dist/css",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/vendors/system-csp-production.js",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/vendors/system-polyfills.js",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/vendors/system-polyfills.src.js",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/vendors/system-csp-production.src.js",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/vendors/system.js",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/vendors/system.src.js",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/components/analytics/ImpactRadius.js",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/components/analytics/ImpactRadius.js.map",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/vendors/npm/assert@1.5.0.js",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/vendors/npm/babel-core@5.8.38.js",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/vendors/npm/base64-js@1.3.0.js",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/vendors/npm/buffer@5.2.1.js",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/vendors/npm/core-js@1.2.7.js",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/vendors/npm/ieee754@1.1.13.js",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/vendors/npm/indexof@0.0.1.js",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/vendors/npm/inherits@2.0.1.js",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/vendors/npm/util@0.10.3.js",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/vendors/npm/path-browserify@0.0.0.js",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/vendors/npm/process@0.11.10.js",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/components/account/CanadaPostAutocomplete.js",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/vendors/npm/vm-browserify@0.0.4.js",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/components/account/CanadaPostAutocomplete.js.map",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/vendors/npm/object-assign@4.1.1.js",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/components/analytics",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/components/account",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/templates/global/woahbar.js",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/templates/global/woahbar.js.map",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/vendors/npm",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/components",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/templates/global",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/vendors",
        "/app_canada_layer/cartridge/static/default/dist/javascripts/templates",
        "/app_canada_layer/cartridge/static/default/dist/javascripts",
        "/app_canada_layer/cartridge/static/default/dist",
        "/app_canada_layer/cartridge/templates/handlebars_compiled/components/account/canadapostautocomplete.js",
        "/app_canada_layer/cartridge/templates/handlebars_compiled/components/account",
        "/app_canada_layer/cartridge/templates/handlebars_compiled/components/analytics/impactradius.js",
        "/app_canada_layer/cartridge/templates/handlebars_compiled/components/analytics",
        "/app_canada_layer/cartridge/templates/handlebars_compiled/components/global/woahbar.js",
        "/app_canada_layer/cartridge/templates/handlebars_compiled/components/global",
        "/app_canada_layer/cartridge/templates/handlebars_compiled/components",
        "/app_canada_layer/cartridge/templates/handlebars_compiled",
    ];

    let vec: Vec<String> = temp_vec.into_iter().map(|item| item.to_string()).collect();

    some_data.remove = Some(vec);

    some_data.update();

    let mut test_data = vec![];
    test_data.push("/app_canada_layer/cartridge/templates/handlebars_compiled".to_owned());
    test_data.push("/app_canada_layer/cartridge/static/default/dist".to_owned());

    assert_eq!(some_data.remove.unwrap(), test_data);
}