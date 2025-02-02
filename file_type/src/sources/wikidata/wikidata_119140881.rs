use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119140881: FileFormat = FileFormat {
    id: 119_140_881,
    source_type: SourceType::Wikidata,
    name: "FreeHand Template 10",
    extensions: &["ft10"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
