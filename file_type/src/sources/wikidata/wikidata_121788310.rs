use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121788310: FileFormat = FileFormat {
    id: 121_788_310,
    source_type: SourceType::Wikidata,
    name: "Microsoft Powerpoint for Macintosh 3",
    extensions: &["ppt"],
    media_types: &["application/vnd.ms-PowerPoint"],
    signatures: &[],
    related_formats: &[],
};
