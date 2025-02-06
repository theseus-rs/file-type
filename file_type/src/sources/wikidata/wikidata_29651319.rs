use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29651319: FileFormat = FileFormat {
    id: 29_651_319,
    source_type: SourceType::Wikidata,
    name: "PixieScript",
    extensions: &["pxs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
