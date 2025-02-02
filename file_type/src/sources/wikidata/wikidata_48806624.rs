use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_48806624: FileFormat = FileFormat {
    id: 48_806_624,
    source_type: SourceType::Wikidata,
    name: "Corel Chart",
    extensions: &["cch"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
