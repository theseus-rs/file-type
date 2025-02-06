use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_115116023: FileFormat = FileFormat {
    id: 115_116_023,
    source_type: SourceType::Wikidata,
    name: "Funpaint Image File",
    extensions: &["fp2", "fun", "vic"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
