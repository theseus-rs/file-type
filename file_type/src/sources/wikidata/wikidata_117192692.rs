use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117192692: FileFormat = FileFormat {
    id: 117_192_692,
    source_type: SourceType::Wikidata,
    name: "Photoshop Raw",
    extensions: &["raw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
