use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116784703: FileFormat = FileFormat {
    id: 116_784_703,
    source_type: SourceType::Wikidata,
    name: "Form Designer Pro Form Contents",
    extensions: &["ofx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
