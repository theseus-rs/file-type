use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116784703: FileFormat = FileFormat {
    id: 116_784_703,
    source_type: SourceType::Wikidata,
    name: "Form Designer Pro Form Contents",
    extensions: &["ofx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
