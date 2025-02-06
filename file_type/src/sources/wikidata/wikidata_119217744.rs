use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119217744: FileFormat = FileFormat {
    id: 119_217_744,
    source_type: SourceType::Wikidata,
    name: "QuickBooks Accountant's Copy Work File",
    extensions: &["qba"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
