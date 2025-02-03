use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117192692: FileFormat = FileFormat {
    id: 117_192_692,
    source_type: SourceType::Wikidata,
    name: "Photoshop Raw",
    extensions: &["raw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
