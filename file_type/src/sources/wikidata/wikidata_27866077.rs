use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27866077: FileFormat = FileFormat {
    id: 27_866_077,
    source_type: SourceType::Wikidata,
    name: "ABC notation, version 2.1",
    extensions: &["abc", "abh"],
    media_types: &["text/vnd.abc"],
    internal_signatures: &[],
    related_formats: &[],
};
