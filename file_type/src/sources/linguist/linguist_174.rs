use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_174: FileType = FileType {
    file_format: &FileFormat {
        id: 174,
        source_type: SourceType::Linguist,
        name: "JSON",
        extensions: &[
            "4DForm",
            "4DProject",
            "JSON-tmLanguage",
            "avsc",
            "geojson",
            "gltf",
            "har",
            "ice",
            "json",
            "json.example",
            "jsonl",
            "mcmeta",
            "sarif",
            "tact",
            "tfstate",
            "tfstate.backup",
            "topojson",
            "webapp",
            "webmanifest",
            "yy",
            "yyp",
        ],
        media_types: &["application/json"],
        signatures: &[],
        related_formats: &[],
    },
};
