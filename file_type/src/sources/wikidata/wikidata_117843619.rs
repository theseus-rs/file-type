use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117843619: FileFormat = FileFormat {
    id: 117_843_619,
    source_type: SourceType::Wikidata,
    name: "Gammalink Gamma Fax file",
    extensions: &["gam"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
