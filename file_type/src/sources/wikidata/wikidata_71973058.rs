use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_71973058: FileFormat = FileFormat {
    id: 71_973_058,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing, version X5",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
