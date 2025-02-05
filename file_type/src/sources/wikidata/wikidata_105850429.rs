use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850429: FileFormat = FileFormat {
    id: 105_850_429,
    source_type: SourceType::Wikidata,
    name: "Cricket Audio XML Bank Description",
    extensions: &["ckbx"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
