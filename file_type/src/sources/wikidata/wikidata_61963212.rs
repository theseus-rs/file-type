use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61963212: FileFormat = FileFormat {
    id: 61_963_212,
    puid: "wikidata/61963212",
    name: "Lotus WordPro Document",
    extensions: &["lwp", "lwp"],
    media_types: &["application/lwp", "application/vnd.lotus-wordpro"],
    internal_signatures: &[],
    related_formats: &[],
};
