use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111431061: FileFormat = FileFormat {
    id: 111_431_061,
    source_type: SourceType::Wikidata,
    name: "C# source code file",
    extensions: &["cs"],
    media_types: &["text/x-csharp"],
    internal_signatures: &[],
    related_formats: &[],
};
