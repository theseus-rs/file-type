use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50308743: FileFormat = FileFormat {
    id: 50_308_743,
    source_type: SourceType::Wikidata,
    name: "WordPerfect file format, v4",
    extensions: &["wp4", "wpd"],
    media_types: &["application/vnd.wordperfect"],
    signatures: &[],
    related_formats: &[],
};
