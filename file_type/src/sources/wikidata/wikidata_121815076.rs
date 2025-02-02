use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121815076: FileFormat = FileFormat {
    id: 121_815_076,
    source_type: SourceType::Wikidata,
    name: "Esko ArtPro File",
    extensions: &["ap"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
