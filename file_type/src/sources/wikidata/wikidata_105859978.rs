use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859978: FileFormat = FileFormat {
    id: 105_859_978,
    source_type: SourceType::Wikidata,
    name: "Visual Studio Visual C++ Project",
    extensions: &["vcxproj"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
