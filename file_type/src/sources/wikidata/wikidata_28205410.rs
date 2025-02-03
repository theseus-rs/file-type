use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205410: FileFormat = FileFormat {
    id: 28_205_410,
    source_type: SourceType::Wikidata,
    name: "Nikon Dust File",
    extensions: &["ndf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
