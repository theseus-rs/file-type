use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130142778: FileType = FileType {
    file_format: &FileFormat {
        id: 130_142_778,
        source_type: SourceType::Wikidata,
        name: "OpenLDAP configuration file",
        extensions: &["ldaprc"],
        media_types: &["text/x-ldapconf"],
        signatures: &[],
        related_formats: &[],
    },
};
