use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121788235: FileFormat = FileFormat {
    id: 121_788_235,
    source_type: SourceType::Wikidata,
    name: "Microsoft Powerpoint for Macintosh 2",
    extensions: &["ppt"],
    media_types: &["application/vnd.ms-PowerPoint"],
    signatures: &[],
    related_formats: &[],
};
