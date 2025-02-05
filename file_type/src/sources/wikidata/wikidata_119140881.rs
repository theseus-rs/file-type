use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119140881: FileFormat = FileFormat {
    id: 119_140_881,
    source_type: SourceType::Wikidata,
    name: "FreeHand Template 10",
    extensions: &["ft10"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
