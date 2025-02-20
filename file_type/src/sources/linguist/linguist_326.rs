use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_326: FileType = FileType {
    file_format: &FileFormat {
        id: 326,
        source_type: SourceType::Linguist,
        name: "Ruby",
        extensions: &[
            "builder",
            "eye",
            "fcgi",
            "gemspec",
            "god",
            "jbuilder",
            "mspec",
            "pluginspec",
            "podspec",
            "prawn",
            "rabl",
            "rake",
            "rb",
            "rbi",
            "rbuild",
            "rbw",
            "rbx",
            "ru",
            "ruby",
            "spec",
            "thor",
            "watchr",
        ],
        media_types: &["text/x-ruby"],
        signatures: &[],
        related_formats: &[],
    },
};
