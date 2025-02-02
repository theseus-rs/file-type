use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59715886: FileFormat = FileFormat {
    id: 59_715_886,
    source_type: SourceType::Wikidata,
    name: "CALS Compressed Bitmap",
    extensions: &["cal"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
