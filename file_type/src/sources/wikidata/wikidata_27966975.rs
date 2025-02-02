use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27966975: FileFormat = FileFormat {
    id: 27_966_975,
    source_type: SourceType::Wikidata,
    name: "WSR",
    extensions: &["wsr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
