use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27473250: FileFormat = FileFormat {
    id: 27_473_250,
    source_type: SourceType::Wikidata,
    name: "Raster Product Format Table of Contents File",
    extensions: &["toc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
