use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121815076: FileFormat = FileFormat {
    id: 121_815_076,
    source_type: SourceType::Wikidata,
    name: "Esko ArtPro File",
    extensions: &["ap"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
