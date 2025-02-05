use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205410: FileFormat = FileFormat {
    id: 28_205_410,
    source_type: SourceType::Wikidata,
    name: "Nikon Dust File",
    extensions: &["ndf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
