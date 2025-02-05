use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_71973058: FileFormat = FileFormat {
    id: 71_973_058,
    source_type: SourceType::Wikidata,
    name: "CorelDraw Drawing, version X5",
    extensions: &["cdr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
