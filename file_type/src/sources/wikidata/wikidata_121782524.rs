use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121782524: FileFormat = FileFormat {
    id: 121_782_524,
    source_type: SourceType::Wikidata,
    name: "Adobe Illustrator CC Artwork 17-23",
    extensions: &["ai", "ait"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
