use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117843750: FileFormat = FileFormat {
    id: 117_843_750,
    source_type: SourceType::Wikidata,
    name: "IBM IOCA",
    extensions: &["ica"],
    media_types: &["image/x-ioca"],
    signatures: &[],
    related_formats: &[],
};
