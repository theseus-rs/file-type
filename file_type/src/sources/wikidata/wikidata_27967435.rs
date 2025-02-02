use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967435: FileFormat = FileFormat {
    id: 27_967_435,
    source_type: SourceType::Wikidata,
    name: "Bink Video, version 2",
    extensions: &["bik2", "bk2"],
    media_types: &["video/vnd.radgamettools.bink"],
    internal_signatures: &[],
    related_formats: &[],
};
