use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27966975: FileFormat = FileFormat {
    id: 27_966_975,
    source_type: SourceType::Wikidata,
    name: "WSR",
    extensions: &["wsr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
