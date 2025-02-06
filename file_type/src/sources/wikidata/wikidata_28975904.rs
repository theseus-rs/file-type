use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975904: FileFormat = FileFormat {
    id: 28_975_904,
    source_type: SourceType::Wikidata,
    name: "Specified points for body pressure file",
    extensions: &["bpi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
