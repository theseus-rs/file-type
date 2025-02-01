use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859978: FileFormat = FileFormat {
    id: 105_859_978,
    puid: "wikidata/105859978",
    name: "Visual Studio Visual C++ Project",
    extensions: &["vcxproj"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
