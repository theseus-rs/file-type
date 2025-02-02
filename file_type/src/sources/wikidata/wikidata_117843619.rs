use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117843619: FileFormat = FileFormat {
    id: 117_843_619,
    source_type: SourceType::Wikidata,
    name: "Gammalink Gamma Fax file",
    extensions: &["gam"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
