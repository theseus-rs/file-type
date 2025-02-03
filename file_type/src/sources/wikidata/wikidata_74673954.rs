use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_74673954: FileFormat = FileFormat {
    id: 74_673_954,
    source_type: SourceType::Wikidata,
    name: "TurboTax return file",
    extensions: &["tax"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
