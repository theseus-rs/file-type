use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27866075: FileFormat = FileFormat {
    id: 27_866_075,
    source_type: SourceType::Wikidata,
    name: "ABC notation, version 1.6",
    extensions: &["abc", "abh"],
    media_types: &["text/vnd.abc"],
    internal_signatures: &[],
    related_formats: &[],
};
