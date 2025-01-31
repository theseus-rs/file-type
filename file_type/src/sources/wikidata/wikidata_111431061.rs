use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111431061: FileFormat = FileFormat {
    id: 111_431_061,
    puid: "wikidata/111431061",
    name: "C# source code file",
    extensions: &["cs"],
    media_types: &["text/x-csharp"],
    internal_signatures: &[],
    related_formats: &[],
};
