use crate::format::FileFormat;

pub(crate) const LINGUIST_131: FileFormat = FileFormat {
    id: 131,
    puid: "linguist/131",
    name: "Gnuplot",
    extensions: &["gnu", "gnuplot", "gp", "p", "plot", "plt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
