use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66439311: FileFormat = FileFormat {
    id: 66_439_311,
    source_type: SourceType::Wikidata,
    name: "Navy DIF",
    extensions: &["dif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
