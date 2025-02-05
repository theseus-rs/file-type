use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61718355: FileFormat = FileFormat {
    id: 61_718_355,
    source_type: SourceType::Wikidata,
    name: "Microsoft PowerPoint for Macintosh, version 4",
    extensions: &["ppt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
