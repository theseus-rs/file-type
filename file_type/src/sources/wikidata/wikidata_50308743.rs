use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_50308743: FileFormat = FileFormat {
    id: 50_308_743,
    source_type: SourceType::Wikidata,
    name: "WordPerfect file format, v4",
    extensions: &["wp4", "wpd"],
    media_types: &["application/vnd.wordperfect"],
    internal_signatures: &[],
    related_formats: &[],
};
