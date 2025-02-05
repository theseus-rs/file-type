use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857574: FileFormat = FileFormat {
    id: 105_857_574,
    source_type: SourceType::Wikidata,
    name: "InstallShield Project",
    extensions: &["ipr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
