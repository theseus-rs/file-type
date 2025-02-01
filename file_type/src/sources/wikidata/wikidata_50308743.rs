use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50308743: FileFormat = FileFormat {
    id: 50_308_743,
    puid: "wikidata/50308743",
    name: "WordPerfect file format, v4",
    extensions: &["wp4", "wpd"],
    media_types: &["application/vnd.wordperfect", "application/vnd.wordperfect"],
    internal_signatures: &[],
    related_formats: &[],
};
