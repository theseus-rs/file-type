use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121788310: FileFormat = FileFormat {
    id: 121_788_310,
    source_type: SourceType::Wikidata,
    name: "Microsoft Powerpoint for Macintosh 3",
    extensions: &["ppt"],
    media_types: &["application/vnd.ms-PowerPoint"],
    internal_signatures: &[],
    related_formats: &[],
};
