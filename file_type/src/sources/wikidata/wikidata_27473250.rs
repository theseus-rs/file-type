use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27473250: FileFormat = FileFormat {
    id: 27_473_250,
    puid: "wikidata/27473250",
    name: "Raster Product Format Table of Contents File",
    extensions: &["toc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
