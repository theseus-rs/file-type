use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111272703: FileFormat = FileFormat {
    id: 111_272_703,
    source_type: SourceType::Wikidata,
    name: "Floating Point raw 32-bit IEEE data",
    extensions: &["f32"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
