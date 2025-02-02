use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122169866: FileFormat = FileFormat {
    id: 122_169_866,
    source_type: SourceType::Wikidata,
    name: "Lotus Notes User ID File",
    extensions: &["id"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
